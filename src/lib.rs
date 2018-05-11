// --- core loge macro, dont call this directly -----------------------------------------
#[macro_export]
macro_rules! loge {
	($lvl: expr, $fmt: expr)				=> (
		println!(concat!($lvl, " [{:30}] > {}"), module_path!(), $fmt)
	);
	($lvl: expr, $fmt: expr, $($args:tt)+)	=> (
		println!(concat!($lvl, " [{:30}] > ", $fmt), module_path!(), $($args)+)
	);
}

// --- debug output (opt in with cfg loge_debug) ----------------------------------------
#[cfg(feature = "loge-debug")]
#[macro_export]
macro_rules! debug {
	($fmt: expr)					=> (loge!("dbug", $fmt));
	($fmt: expr, $($args:tt)+)		=> (loge!("dbug", $fmt, $($args)+));
}
#[cfg(not(feature = "loge-debug"))]
#[macro_export]
macro_rules! debug {
	($($_: tt)*) => { };
}

// --- info output (opt out with cfg no_loge_info) --------------------------------------
#[macro_export]
#[cfg(not(feature = "no-loge-info"))]
macro_rules! info {
	($fmt: expr)					=> (loge!("info", $fmt));
	($fmt: expr, $($args:tt)+)		=> (loge!("info", $fmt, $($args)+));
}

#[macro_export]
#[cfg(feature = "no-loge-info")]
macro_rules! info {
	($($_: tt)*) => { };
}

// --- warn output (always on) ----------------------------------------------------------
#[macro_export]
macro_rules! warn {
	($fmt: expr)					=> (loge!("WARN", $fmt));
	($fmt: expr, $($args:tt)+)		=> (loge!("WARN", $fmt, $($args)+));
}

// --- tests ----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    #[test]
    fn loge() {
		debug!("wow, such debug");

		let s = String::from("so doge");
		info!("the string {:?} is {} characters long", s, s.len());

		info!(String::from("print a string"));

		let s2 = String::from("another string");
		info!(s2);

		warn!("much warning");
    }
}
