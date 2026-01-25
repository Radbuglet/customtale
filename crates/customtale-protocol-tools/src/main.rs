use std::env;

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
        .build()?;

    // Discover all packet types in the registry
    let packet_types = discover_packet_types(&jvm)?;

    dbg!(packet_types.len());

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
        packet_types.push(ty);
    }

    Ok(packet_types)
}
