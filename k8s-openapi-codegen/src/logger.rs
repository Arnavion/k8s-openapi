pub(crate) fn make_local_logger(version_name: &'static str) -> env_logger::Logger {
	let mut builder = env_logger::Builder::new();
	builder.format(move |buf, record| {
		use std::io::Write;
		writeln!(buf, "[{}] {} {}:{} {}", version_name, record.level(), record.file().unwrap_or("?"), record.line().unwrap_or(0), record.args())
	});
	let rust_log = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
	builder.parse_filters(&rust_log).build()
}

pub(crate) struct Logger;

impl log::Log for Logger {
	fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
		TASK_LOCAL_LOGGER.try_with(|logger| logger.enabled(metadata)).unwrap_or_else(|_|
			THREAD_LOCAL_LOGGER.with(|thread_local_logger| thread_local_logger.borrow().as_ref().map(|logger| logger.enabled(metadata)).unwrap_or_default()))
	}

	fn log(&self, record: &log::Record<'_>) {
		TASK_LOCAL_LOGGER.try_with(|logger| logger.log(record)).unwrap_or_else(|_|
			THREAD_LOCAL_LOGGER.with(|thread_local_logger| if let Some(logger) = thread_local_logger.borrow().as_ref() { logger.log(record); }));
	}

	fn flush(&self) {
		TASK_LOCAL_LOGGER.try_with(log::Log::flush).unwrap_or_else(|_|
			THREAD_LOCAL_LOGGER.with(|thread_local_logger| if let Some(logger) = thread_local_logger.borrow().as_ref() { logger.flush(); }));
	}
}

tokio::task_local! {
	pub(crate) static TASK_LOCAL_LOGGER: env_logger::Logger;
}

pub(crate) fn register_thread_local_logger(logger: env_logger::Logger) {
	THREAD_LOCAL_LOGGER.with(|thread_local_logger| *thread_local_logger.borrow_mut() = Some(logger));
}

thread_local! {
	static THREAD_LOCAL_LOGGER: std::cell::RefCell<Option<env_logger::Logger>> = std::cell::RefCell::new(None);
}
