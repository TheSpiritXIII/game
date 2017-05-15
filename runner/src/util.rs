use chrono::prelude::*;

pub fn print_time()
{
	let local_time = Local::now();
	print!("[{} {:04}] ", local_time.format("%r"), local_time.timestamp_subsec_millis());
}

pub fn as_answer(value: bool) -> &'static str
{
	if value
	{
		"yes"
	}
	else
	{
		"no"
	}
}

pub fn as_status(value: bool) -> &'static str
{
	if value
	{
		"enabled"
	}
	else
	{
		"disabled"
	}
}
