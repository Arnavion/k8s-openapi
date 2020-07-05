pub(crate) struct Logger;

impl log::Log for Logger {
	fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
		THREAD_LOCAL_LOGGER.with(|thread_local_logger|
			if let Some(logger) = thread_local_logger.borrow().as_ref() { logger.enabled(metadata) } else { false })
	}

	fn log(&self, record: &log::Record<'_>) {
		THREAD_LOCAL_LOGGER.with(|thread_local_logger| if let Some(logger) = thread_local_logger.borrow().as_ref() { logger.log(record); })
	}

	fn flush(&self) {
		THREAD_LOCAL_LOGGER.with(|thread_local_logger| if let Some(logger) = thread_local_logger.borrow().as_ref() { logger.flush(); })
	}
}

pub(crate) fn register_thread_local_logger(logger: env_logger::Logger) {
	THREAD_LOCAL_LOGGER.with(|thread_local_logger| *thread_local_logger.borrow_mut() = Some(logger));
}

thread_local! {
	static THREAD_LOCAL_LOGGER: std::cell::RefCell<Option<env_logger::Logger>> = std::cell::RefCell::new(None);
}
