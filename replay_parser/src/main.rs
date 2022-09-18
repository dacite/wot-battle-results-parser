use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use wot_replay_parser::events::BattleEvent;
pub fn main() {
    let subscriber = FmtSubscriber::builder().with_max_level(Level::TRACE).finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    // let formatting_layer = BunyanFormattingLayer::new(
    //     "wot_replay_parser".into(),
    //     // Output the formatted spans to stdout.
    //     std::io::stdout,
    // );
    // // The `with` method is provided by `SubscriberExt`, an extension
    // // trait for `Subscriber` exposed by `tracing_subscriber`
    // let subscriber = Registry::default().with(JsonStorageLayer).with(formatting_layer);
    // // `set_global_default` can be used by applications to specify
    // // what subscriber should be used to process spans.
    // set_global_default(subscriber).expect("Failed to set subscriber");

    let paths = utils::parse_dir("/home/dacite/Projects/wot-battle-results-parser/replays").unwrap();
    for path in paths {
        print_chat(path.path().as_os_str().to_str().unwrap())
    }
    let path = "/home/dacite/Projects/wot-battle-results-parser/replay_parser/input_files/20200502_0355_sweden-S26_Lansen_C_06_ensk.wotreplay";
    print_chat(path)
}
#[tracing::instrument]
fn print_chat(path: &str) {
    let parser = wot_replay_parser::ReplayParser::parse_file(path).unwrap();

    for event in parser.event_stream().unwrap().flatten() {
        if let BattleEvent::AvatarCreate(avatar) = event.into_battle_event() {
            println!("{:?}", avatar);
            return;
        }
    }
    todo!()
}
