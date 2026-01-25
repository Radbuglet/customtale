use std::env;

use anyhow::Context;
use j4rs::{InvocationArg, JvmBuilder};

fn main() -> anyhow::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let args = args.iter().map(|v| v.as_str()).collect::<Vec<_>>();
    let [_exe, hytale_jar] = &args[..] else {
        anyhow::bail!("bad usage");
    };

    let jvm = JvmBuilder::new()
        .classpath_entry(j4rs::ClasspathEntry::new(hytale_jar))
        .java_opt(j4rs::JavaOpt::new("--enable-native-access=ALL-UNNAMED"))
        .build()
        .context("failed to initialize JVM")?;

    // Discover all packet types in the registry
    let packet_types = discover_packet_types(&jvm).context("failed to enumerate packet types")?;

    for ty in &packet_types {
        let name = jvm.to_rust::<String>(jvm.invoke(ty, "getName", &no_args())?)?;

        println!("Testing {name}...");
        let instance =
            randomize_instance(&jvm, ty).context("failed to construct randomized instance")?;
    }

    Ok(())
}

fn init_logger() {
    unsafe { env::set_var("J4RS_CONSOLE_LOG_LEVEL", "debug") };

    env_logger::builder()
        .filter(None, log::LevelFilter::Info)
        .init();
}

fn no_args() -> [InvocationArg; 0] {
    []
}

fn discover_packet_types(jvm: &j4rs::Jvm) -> anyhow::Result<Vec<j4rs::Instance>> {
    let packets = jvm.invoke_static(
        "com.hypixel.hytale.protocol.PacketRegistry",
        "all",
        &no_args(),
    )?;

    let packets = jvm.invoke(&packets, "values", &no_args())?;
    let packets = jvm.invoke(&packets, "iterator", &no_args())?;
    let mut packet_types = Vec::new();

    loop {
        let has_next = jvm.to_rust::<bool>(jvm.invoke(&packets, "hasNext", &no_args())?)?;

        if !has_next {
            break;
        }

        let packet = jvm.invoke(&packets, "next", &no_args())?;
        let packet = jvm.cast(
            &packet,
            "com.hypixel.hytale.protocol.PacketRegistry$PacketInfo",
        )?;

        let clazz = jvm.invoke(&packet, "getClass", &no_args())?;
        let field = &jvm.invoke(
            &clazz,
            "getDeclaredField",
            &[InvocationArg::try_from("type").unwrap()],
        )?;

        jvm.invoke(
            field,
            "setAccessible",
            &[InvocationArg::try_from(true).unwrap().into_primitive()?],
        )?;

        let ty = jvm.invoke(field, "get", &[InvocationArg::from(packet)])?;
        let ty = jvm.cast(&ty, "java.lang.Class")?;

        packet_types.push(ty);
    }

    Ok(packet_types)
}

fn randomize_instance(jvm: &j4rs::Jvm, clazz: &j4rs::Instance) -> anyhow::Result<j4rs::Instance> {
    let name = jvm.to_rust::<String>(jvm.invoke(clazz, "getName", &no_args())?)?;

    if name.starts_with("com.hypixel.hytale.protocol") {
        return randomize_instance_hytale(jvm, clazz);
    }

    unreachable!()
}

fn randomize_instance_hytale(
    jvm: &j4rs::Jvm,
    clazz: &j4rs::Instance,
) -> anyhow::Result<j4rs::Instance> {
    let ctors = jvm.invoke(clazz, "getConstructors", &no_args())?;

    for ctor in marshal_array(jvm, ctors, "java.lang.reflect.Constructor")? {
        let param_types = marshal_array(
            jvm,
            jvm.invoke(&ctor, "getParameterTypes", &no_args())?,
            "java.lang.Class",
        )?;

        if param_types.is_empty() {
            continue;
        }

        if let [param_ty] = &param_types[..]
            && jvm.check_equals(clazz, InvocationArg::from(jvm.clone_instance(param_ty)?))?
        {
            continue;
        }

        for ty in &param_types {
            dbg!(jvm.to_rust::<String>(jvm.invoke(ty, "toString", &no_args())?)?);
        }
    }

    // TODO
    Ok(jvm.clone_instance(clazz)?)
}

fn marshal_array(
    jvm: &j4rs::Jvm,
    target: j4rs::Instance,
    ty: &str,
) -> anyhow::Result<Vec<j4rs::Instance>> {
    let target = jvm.invoke_static("java.util.Arrays", "asList", &[InvocationArg::from(target)])?;
    let target = jvm.invoke(&target, "iterator", &no_args())?;

    let mut collector = Vec::new();

    loop {
        let has_next = jvm.to_rust::<bool>(jvm.invoke(&target, "hasNext", &no_args())?)?;

        if !has_next {
            break;
        }

        let elem = jvm.invoke(&target, "next", &no_args())?;
        let elem = jvm.cast(&elem, ty)?;

        collector.push(elem);
    }

    Ok(collector)
}
