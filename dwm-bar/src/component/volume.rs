// This implementation is copy from https://github.com/krruzic/pulsectl

extern crate libpulse_binding as pulse;

use std::{cell::RefCell, ops::Deref, rc::Rc};

use super::widget::Block;
use pulse::{
    callbacks::ListResult,
    context::{introspect, Context as PulseContext},
    mainloop::standard::{IterateResult, Mainloop},
    operation::{Operation, State},
    proplist::Proplist,
    volume::ChannelVolumes,
};

use anyhow::{anyhow, Context, Result};

struct Device {
    volume: ChannelVolumes,
}

impl<'a> From<&'a introspect::SinkInfo<'a>> for Device {
    fn from(item: &'a introspect::SinkInfo<'a>) -> Self {
        Self {
            volume: item.volume,
        }
    }
}

struct Server {
    default_sink_name: Option<String>,
}

impl<'a> From<&'a introspect::ServerInfo<'a>> for Server {
    fn from(info: &'a introspect::ServerInfo<'a>) -> Self {
        Self {
            default_sink_name: info.default_sink_name.as_ref().map(|cow| cow.to_string()),
        }
    }
}

struct SinkController {
    mainloop: Rc<RefCell<Mainloop>>,
    context: Rc<RefCell<PulseContext>>,
    introspect: introspect::Introspector,
}

impl Drop for SinkController {
    fn drop(&mut self) {
        self.context.borrow_mut().disconnect();
        self.mainloop.borrow_mut().quit(pulse::def::Retval(0));
    }
}

impl SinkController {
    pub fn new() -> Result<SinkController> {
        let mut proplist = Proplist::new().unwrap();
        proplist
            .set_str(
                pulse::proplist::properties::APPLICATION_NAME,
                "DWMBarVolumeFetcher",
            )
            .unwrap();

        let mainloop = Rc::new(RefCell::new(
            Mainloop::new().ok_or_else(|| anyhow!("Fail to create mainloop"))?,
        ));
        let context = Rc::new(RefCell::new(
            PulseContext::new_with_proplist(mainloop.borrow().deref(), "MainConn", &proplist)
                .ok_or_else(|| anyhow!("Fail to create context"))?,
        ));
        context
            .borrow_mut()
            .connect(None, pulse::context::FlagSet::NOFLAGS, None)
            .context("Fail to connect to context")?;

        loop {
            match mainloop.borrow_mut().iterate(false) {
                IterateResult::Err(e) => {
                    eprintln!("Fail to iterate mainloop");
                    return Err(anyhow!(e));
                }
                IterateResult::Quit(_) => {
                    eprintln!("Fail to iterate mainloop, without error...");
                    return Err(anyhow!("Mainloop iterate quit without an error"));
                }
                _ => {}
            }

            use pulse::context::State::{Failed, Ready, Terminated};
            match context.borrow_mut().get_state() {
                Ready => break,
                Failed | Terminated => {
                    eprintln!("Connection failed, or terminated");
                    return Err(anyhow!(
                        "Connection context failed or terminated without error"
                    ));
                }
                _ => {}
            }
        }

        let introspect = context.borrow_mut().introspect();

        Ok(SinkController {
            mainloop,
            context,
            introspect,
        })
    }

    fn wait_for_operation<G: ?Sized>(&mut self, op: Operation<G>) -> Result<()> {
        loop {
            match self.mainloop.borrow_mut().iterate(false) {
                IterateResult::Err(e) => return Err(e.into()),
                IterateResult::Success(_) => {}
                IterateResult::Quit(_) => {
                    return Err(anyhow!("mainloop quit without an error",));
                }
            }
            match op.get_state() {
                State::Done => {
                    break;
                }
                State::Running => {}
                State::Cancelled => {
                    return Err(anyhow!("Operation cancelled without an error",));
                }
            }
        }
        Ok(())
    }

    fn get_server_info(&mut self) -> Result<Server> {
        let server = Rc::new(RefCell::new(Some(None)));
        let r_server = server.clone();
        let op = self.introspect.get_server_info(move |res| {
            r_server.borrow_mut().as_mut().unwrap().replace(res.into());
        });
        self.wait_for_operation(op)?;
        let mut result = server.borrow_mut();
        Ok(result
            .take()
            .unwrap()
            .ok_or_else(|| anyhow!("Error getting information about the server"))?)
    }

    fn get_default_device(&mut self) -> Result<Device> {
        let device = Rc::new(RefCell::new(Some(None)));
        let r_dev = device.clone();
        let name = self.get_server_info()?.default_sink_name;
        match name {
            Some(n) => {
                let op = self.introspect.get_sink_info_by_name(
                    n.as_str(),
                    move |sinks: ListResult<&introspect::SinkInfo>| {
                        if let ListResult::Item(item) = sinks {
                            r_dev.borrow_mut().as_mut().unwrap().replace(item.into());
                        }
                    },
                );

                self.wait_for_operation(op)?;
                let mut result = device.borrow_mut();
                result
                    .take()
                    .unwrap()
                    .ok_or_else(|| anyhow!("Fail to get default device information"))
            }

            None => Err(anyhow!(
                "Fail to get default sink name due to server info missing"
            )),
        }
    }
}

#[test]
fn test() {
    let mut s = SinkController::new().unwrap();
    let dev = s.get_default_device();
    dbg!(dev.unwrap().volume.print());
}

/// Create a sound volume component for bar
pub async fn sound_volume() -> Option<Block> {
    let mut s = SinkController::new().ok()?;
    let dev = s.get_default_device().ok()?;
    let devices = dev.volume.get();
    if devices.is_empty() {
        return None;
    }
    let dev = devices[0];
    let icon = if dev.is_muted() { "" } else { "" };
    Some(
        Block::new(icon, format!("{}%", dev.print().trim()))
            .text_fg("#EAEAEA")
            .icon_fg("#EAEAEA"),
    )
}
