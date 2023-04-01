use flexi_logger::{
    Age, Cleanup, Criterion, Duplicate, FileSpec, FlexiLoggerError, LevelFilter, LogSpecBuilder,
    Logger, Naming,
};

// Blatantly stol... borrowed from https://github.com/jac3km4/redscript
// Both projects are MIT licensed with the same original author, jekky
/// # Errors
/// Returns `FlexiLoggerError`
pub fn setup<P: AsRef<std::path::Path>>(logs_dir: P) -> Result<(), FlexiLoggerError> {
    let file = FileSpec::default()
        .directory(logs_dir.as_ref())
        .basename("cybercmd");
    let logger = Logger::with(LogSpecBuilder::new().default(LevelFilter::Info).build())
        .log_to_file(file)
        .duplicate_to_stdout(Duplicate::All)
        .rotate(
            Criterion::Age(Age::Day),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(4),
        )
        .format(|out, time, msg| {
            write!(
                out,
                "[{} - {}] {}",
                msg.level(),
                time.now().to_rfc2822(),
                msg.args()
            )
        });
    logger.start()?;
    Ok(())
}
