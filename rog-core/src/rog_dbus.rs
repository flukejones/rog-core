use crate::config::Config;
use crate::daemon::DbusU8Type;
use dbus::tree::{Factory, MTSync, Method, MethodErr, Signal, Tree};
use log::warn;
use rog_client::{aura_modes::AuraModes, DBUS_IFACE, DBUS_PATH};
use std::sync::Arc;
use tokio::sync::{
    mpsc::{channel, Receiver, Sender},
    Mutex,
};

pub(super) fn dbus_set_ledmsg(sender: Mutex<Sender<AuraModes>>) -> Method<MTSync, ()> {
    let factory = Factory::new_sync::<()>();
    factory
        // method for ledmessage
        .method("SetKeyBacklight", (), {
            move |m| {
                let json: &str = m.msg.read1()?;
                if let Ok(mut lock) = sender.try_lock() {
                    if let Ok(data) = serde_json::from_str(json) {
                        lock.try_send(data).unwrap_or_else(|err| {
                            warn!("SetKeyBacklight over mpsc failed: {}", err)
                        });
                    } else {
                        warn!("SetKeyBacklight could not deserialise");
                    }
                    Ok(vec![])
                } else {
                    Err(MethodErr::failed("Could not lock daemon for access"))
                }
            }
        })
        .inarg::<&str, _>("json")
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
                    Ok(vec![])
                } else {
                    Err(MethodErr::failed("Could not lock daemon for access"))
                }
            }
        })
        .inarg::<u8, _>("byte")
        .annotate("org.freedesktop.DBus.Method.NoReply", "true")
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
                    Ok(vec![])
                } else {
                    Err(MethodErr::failed("Could not lock daemon for access"))
                }
            }
        })
        .inarg::<u8, _>("byte")
        .annotate("org.freedesktop.DBus.Method.NoReply", "true")
}

#[allow(clippy::type_complexity)]
pub(super) fn dbus_create_tree(
    config: Arc<Mutex<Config>>,
) -> (
    Tree<MTSync, ()>,
    Sender<AuraModes>,
    Receiver<AuraModes>,
    Receiver<Vec<Vec<u8>>>,
    DbusU8Type,
    DbusU8Type,
    Arc<Signal<()>>,
    Arc<Signal<()>>,
    Arc<Signal<()>>,
) {
    let (aura_command_send, aura_command_recv) = channel::<AuraModes>(1);
    let (animatrix_send, animatrix_recv) = channel::<Vec<Vec<u8>>>(1);
    let fan_mode: DbusU8Type = Arc::new(Mutex::new(None));
    let charge_limit: DbusU8Type = Arc::new(Mutex::new(None));

    let factory = Factory::new_sync::<()>();

    let builtin_mode_sig = Arc::new(
        factory
            .signal("KeyBacklightChanged", ())
            .sarg::<bool, _>("value"),
    );
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
                    .add_m(dbus_set_animatrix(Mutex::new(animatrix_send)))
                    .add_m(dbus_set_fan_mode(fan_mode.clone()))
                    .add_m(dbus_set_charge_limit(charge_limit.clone()))
                    .add_m(dbus_get_fan_mode(config.clone()))
                    .add_m(dbus_get_charge_limit(config))
                    .add_s(builtin_mode_sig.clone())
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
        builtin_mode_sig,
        fanmode_changed_sig,
        chrg_limit_changed_sig,
    )
}
