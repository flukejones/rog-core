use crate::config::Config;
use crate::daemon::DbusU8Type;
use crate::led_control::AuraCommand;
use crate::rogcore::FanLevel;
use dbus::tree::{Factory, MTSync, Method, MethodErr, Signal, Tree};
use log::warn;
use rog_client::{DBUS_IFACE, DBUS_PATH};
use std::sync::Arc;
use tokio::sync::{
    mpsc::{channel, Receiver, Sender},
    Mutex,
};

pub(super) fn dbus_set_ledmsg(sender: Mutex<Sender<AuraCommand>>) -> Method<MTSync, ()> {
    let factory = Factory::new_sync::<()>();
    factory
        // method for ledmessage
        .method("LedWriteBytes", (), {
            move |m| {
                let bytes: Vec<u8> = m.msg.read1()?;
                if let Ok(mut lock) = sender.try_lock() {
                    let command = AuraCommand::WriteBytes(bytes.to_vec());
                    lock.try_send(command)
                        .unwrap_or_else(|err| warn!("LedWriteBytes over mpsc failed: {}", err));
                    let mret = m
                        .msg
                        .method_return()
                        .append1(&format!("Wrote {:x?}", bytes));
                    Ok(vec![mret])
                } else {
                    Err(MethodErr::failed("Could not lock daemon for access"))
                }
            }
        })
        .outarg::<&str, _>("reply")
        .inarg::<Vec<u8>, _>("bytearray")
}

pub(super) fn dbus_set_ledmultizone(sender: Mutex<Sender<AuraCommand>>) -> Method<MTSync, ()> {
    let factory = Factory::new_sync::<()>();
    factory
        // method for ledmessage
        .method("LedWriteMultizone", (), {
            move |m| {
                if let Ok(mut lock) = sender.try_lock() {
                    let mut iter = m.msg.iter_init();
                    let byte_array: Vec<Vec<u8>> =
                        vec![iter.read()?, iter.read()?, iter.read()?, iter.read()?];
                    let command = AuraCommand::WriteMultizone(byte_array);
                    lock.try_send(command)
                        .unwrap_or_else(|err| warn!("LedWriteMultizone over mpsc failed: {}", err));
                    let mret = m
                        .msg
                        .method_return()
                        .append1(&"Got effect part".to_string());
                    Ok(vec![mret])
                } else {
                    Err(MethodErr::failed("Could not lock daemon for access"))
                }
            }
        })
        .outarg::<&str, _>("reply")
        .inarg::<Vec<u8>, _>("bytearray1")
        .inarg::<Vec<u8>, _>("bytearray2")
        .inarg::<Vec<u8>, _>("bytearray3")
        .inarg::<Vec<u8>, _>("bytearray4")
        .annotate("org.freedesktop.DBus.Method.NoReply", "true")
}

pub(super) fn dbus_set_ledeffect(sender: Mutex<Sender<AuraCommand>>) -> Method<MTSync, ()> {
    let factory = Factory::new_sync::<()>();
    factory
        // method for ledmessage
        .method("LedWriteEffect", (), {
            move |m| {
                if let Ok(mut lock) = sender.try_lock() {
                    let mut iter = m.msg.iter_init();
                    let byte_array: Vec<Vec<u8>> = vec![
                        iter.read()?,
                        iter.read()?,
                        iter.read()?,
                        iter.read()?,
                        iter.read()?,
                        iter.read()?,
                        iter.read()?,
                        iter.read()?,
                        iter.read()?,
                        iter.read()?,
                        iter.read()?,
                    ];
                    let command = AuraCommand::WriteEffect(byte_array);
                    lock.try_send(command)
                        .unwrap_or_else(|err| warn!("LedWriteEffect over mpsc failed: {}", err));
                    Ok(vec![])
                } else {
                    Err(MethodErr::failed("Could not lock daemon for access"))
                }
            }
        })
        .inarg::<Vec<u8>, _>("bytearray1")
        .inarg::<Vec<u8>, _>("bytearray2")
        .inarg::<Vec<u8>, _>("bytearray3")
        .inarg::<Vec<u8>, _>("bytearray4")
        .inarg::<Vec<u8>, _>("bytearray5")
        .inarg::<Vec<u8>, _>("bytearray6")
        .inarg::<Vec<u8>, _>("bytearray7")
        .inarg::<Vec<u8>, _>("bytearray8")
        .inarg::<Vec<u8>, _>("bytearray9")
        .inarg::<Vec<u8>, _>("bytearray10")
        .inarg::<Vec<u8>, _>("bytearray11")
        .annotate("org.freedesktop.DBus.Method.NoReply", "true")
}

pub(super) fn dbus_set_animatrix(
    sender: Mutex<Sender<Vec<Vec<u8>>>>, // need mutex only to get interior mutability in MTSync
) -> Method<MTSync, ()> {
    let factory = Factory::new_sync::<()>();
    factory
        // method for ledmessage
        .method("AnimatrixWrite", (), {
            move |m| {
                let mut iter = m.msg.iter_init();
                let byte_array: Vec<Vec<u8>> = vec![iter.read()?, iter.read()?];
                if let Ok(mut lock) = sender.try_lock() {
                    // Ignore errors if the channel is already full
                    lock.try_send(byte_array).unwrap_or_else(|_err| {});
                    Ok(vec![])
                } else {
                    Err(MethodErr::failed("Could not lock daemon for access"))
                }
            }
        })
        .inarg::<Vec<u8>, _>("bytearray1")
        .inarg::<Vec<u8>, _>("bytearray2")
        .annotate("org.freedesktop.DBus.Method.NoReply", "true")
}

pub(super) fn dbus_set_fan_mode(data: DbusU8Type) -> Method<MTSync, ()> {
    let factory = Factory::new_sync::<()>();
    factory
        // method for ledmessage
        .method("SetFanMode", (), {
            move |m| {
                if let Ok(mut lock) = data.try_lock() {
                    let mut iter = m.msg.iter_init();
                    let byte: u8 = iter.read()?;
                    *lock = Some(byte);
                    let mret = m
                        .msg
                        .method_return()
                        .append1(format!("Fan level set to {:?}", FanLevel::from(byte)));
                    Ok(vec![mret])
                } else {
                    Err(MethodErr::failed("Could not lock daemon for access"))
                }
            }
        })
        .outarg::<&str, _>("reply")
        .inarg::<u8, _>("byte")
}

pub(super) fn dbus_get_fan_mode(config: Arc<Mutex<Config>>) -> Method<MTSync, ()> {
    let factory = Factory::new_sync::<()>();
    factory
        .method("GetFanMode", (), {
            move |m| {
                if let Ok(lock) = config.try_lock() {
                    let mret = m.msg.method_return().append1(lock.fan_mode);
                    Ok(vec![mret])
                } else {
                    Err(MethodErr::failed("Could not lock config for access"))
                }
            }
        })
        .outarg::<&str, _>("value")
}

pub(super) fn dbus_get_charge_limit(config: Arc<Mutex<Config>>) -> Method<MTSync, ()> {
    let factory = Factory::new_sync::<()>();
    factory
        .method("GetChargeLimit", (), {
            move |m| {
                if let Ok(lock) = config.try_lock() {
                    let mret = m.msg.method_return().append1(lock.bat_charge_limit);
                    Ok(vec![mret])
                } else {
                    Err(MethodErr::failed("Could not lock config for access"))
                }
            }
        })
        .outarg::<&str, _>("value")
}

pub(super) fn dbus_set_charge_limit(data: DbusU8Type) -> Method<MTSync, ()> {
    let factory = Factory::new_sync::<()>();
    factory
        // method for ledmessage
        .method("SetChargeLimit", (), {
            move |m| {
                if let Ok(mut lock) = data.try_lock() {
                    let mut iter = m.msg.iter_init();
                    let byte: u8 = iter.read()?;
                    *lock = Some(byte);
                    let mret = m
                        .msg
                        .method_return()
                        .append1(format!("Battery charge limit set to {}", byte));
                    Ok(vec![mret])
                } else {
                    Err(MethodErr::failed("Could not lock daemon for access"))
                }
            }
        })
        .outarg::<&str, _>("reply")
        .inarg::<u8, _>("byte")
}

#[allow(clippy::type_complexity)]
pub(super) fn dbus_create_tree(
    config: Arc<Mutex<Config>>,
) -> (
    Tree<MTSync, ()>,
    Sender<AuraCommand>,
    Receiver<AuraCommand>,
    Receiver<Vec<Vec<u8>>>,
    DbusU8Type,
    DbusU8Type,
    Arc<Signal<()>>,
    Arc<Signal<()>>,
    Arc<Signal<()>>,
) {
    let (aura_command_send, aura_command_recv) = channel::<AuraCommand>(1);
    let (animatrix_send, animatrix_recv) = channel::<Vec<Vec<u8>>>(1);
    let fan_mode: DbusU8Type = Arc::new(Mutex::new(None));
    let charge_limit: DbusU8Type = Arc::new(Mutex::new(None));

    let factory = Factory::new_sync::<()>();

    let effect_cancel_sig = Arc::new(factory.signal("LedCancelEffect", ()));
    let fanmode_changed_sig = Arc::new(factory.signal("FanModeChanged", ()).sarg::<u8, _>("value"));
    let chrg_limit_changed_sig = Arc::new(
        factory
            .signal("ChargeLimitChanged", ())
            .sarg::<u8, _>("value"),
    );

    let tree = factory
        .tree(())
        .add(
            factory.object_path(DBUS_PATH, ()).introspectable().add(
                factory
                    .interface(DBUS_IFACE, ())
                    .add_m(dbus_set_ledmsg(Mutex::new(aura_command_send.clone())))
                    .add_m(dbus_set_ledmultizone(Mutex::new(aura_command_send.clone())))
                    .add_m(dbus_set_ledeffect(Mutex::new(aura_command_send.clone())))
                    .add_m(dbus_set_animatrix(Mutex::new(animatrix_send)))
                    .add_m(dbus_set_fan_mode(fan_mode.clone()))
                    .add_m(dbus_set_charge_limit(charge_limit.clone()))
                    .add_m(dbus_get_fan_mode(config.clone()))
                    .add_m(dbus_get_charge_limit(config.clone()))
                    .add_s(effect_cancel_sig.clone())
                    .add_s(fanmode_changed_sig.clone())
                    .add_s(chrg_limit_changed_sig.clone()),
            ),
        )
        .add(factory.object_path("/", ()).introspectable());
    (
        tree,
        aura_command_send,
        aura_command_recv,
        animatrix_recv,
        fan_mode,
        charge_limit,
        effect_cancel_sig,
        fanmode_changed_sig,
        chrg_limit_changed_sig,
    )
}
