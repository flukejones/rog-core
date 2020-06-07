use crate::daemon::{FanModeType, LedMsgType, NestedVecType};
use crate::rogcore::FanLevel;
use dbus::tree::{Factory, MTSync, Method, MethodErr, Signal, Tree};
use rog_client::{DBUS_IFACE, DBUS_PATH};
use std::sync::Arc;
use tokio::sync::Mutex;

pub(super) fn dbus_create_ledmsg_method(msg: LedMsgType) -> Method<MTSync, ()> {
    let factory = Factory::new_sync::<()>();
    factory
        // method for ledmessage
        .method("LedWriteBytes", (), {
            move |m| {
                let bytes: Vec<u8> = m.msg.read1()?;
                if let Ok(mut lock) = msg.try_lock() {
                    *lock = Some(bytes.to_vec());
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

pub(super) fn dbus_create_ledmultizone_method(effect: NestedVecType) -> Method<MTSync, ()> {
    let factory = Factory::new_sync::<()>();
    factory
        // method for ledmessage
        .method("LedWriteMultizone", (), {
            move |m| {
                if let Ok(mut lock) = effect.try_lock() {
                    let mut iter = m.msg.iter_init();
                    let byte_array: Vec<Vec<u8>> =
                        vec![iter.read()?, iter.read()?, iter.read()?, iter.read()?];
                    *lock = Some(byte_array);
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

pub(super) fn dbus_create_ledeffect_method(effect: NestedVecType) -> Method<MTSync, ()> {
    let factory = Factory::new_sync::<()>();
    factory
        // method for ledmessage
        .method("LedWriteEffect", (), {
            move |m| {
                if let Ok(mut lock) = effect.try_lock() {
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
                    *lock = Some(byte_array);
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

pub(super) fn dbus_create_animatrix_method(sender: Arc<Mutex<tokio::sync::mpsc::Sender<Vec<Vec<u8>>>>>) -> Method<MTSync, ()> {
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

pub(super) fn dbus_create_fan_mode_method(fan_mode: FanModeType) -> Method<MTSync, ()> {
    let factory = Factory::new_sync::<()>();
    factory
        // method for ledmessage
        .method("FanMode", (), {
            move |m| {
                if let Ok(mut lock) = fan_mode.try_lock() {
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

pub(super) fn dbus_create_tree() -> (
    Tree<MTSync, ()>,
    LedMsgType,
    NestedVecType,
    tokio::sync::mpsc::Receiver<Vec<Vec<u8>>>,
    FanModeType,
    Arc<Signal<()>>,
) {
    let input_bytes: LedMsgType = Arc::new(Mutex::new(None));
    let input_effect: NestedVecType = Arc::new(Mutex::new(None));
    let (animatrix_send, animatrix_recv) = tokio::sync::mpsc::channel::<Vec<Vec<u8>>>(1);
    let fan_mode: FanModeType = Arc::new(Mutex::new(None));

    let factory = Factory::new_sync::<()>();
    let effect_cancel_sig = Arc::new(factory.signal("LedCancelEffect", ()));
    let tree = factory
        .tree(())
        .add(
            factory.object_path(DBUS_PATH, ()).introspectable().add(
                factory
                    .interface(DBUS_IFACE, ())
                    .add_m(dbus_create_ledmsg_method(input_bytes.clone()))
                    .add_m(dbus_create_ledmultizone_method(input_effect.clone()))
                    .add_m(dbus_create_ledeffect_method(input_effect.clone()))
                    .add_m(dbus_create_animatrix_method(Arc::new(Mutex::new(animatrix_send))))
                    .add_m(dbus_create_fan_mode_method(fan_mode.clone()))
                    .add_s(effect_cancel_sig.clone()),
            ),
        )
        .add(factory.object_path("/", ()).introspectable());
    (
        tree,
        input_bytes,
        input_effect,
        animatrix_recv,
        fan_mode,
        effect_cancel_sig,
    )
}
