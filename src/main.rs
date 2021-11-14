use sqlx::postgres::PgPool;
use std::env;
use sqlx::types::BigDecimal;
use sysinfo::{ProcessExt, System, SystemExt};
use num_traits::cast::FromPrimitive;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
	if env::var("DATABASE_URL").is_err() {
		eprintln!("The environment variable DATABASE_URL is required");
		std::process::exit(-1);
	}
	let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

	sqlx::migrate!().run(&pool).await?;

	let mut sys = System::new_all();
	sys.refresh_all();

	let sysid = uuid::Uuid::new_v4();
	let now = chrono::NaiveDateTime::from_timestamp(chrono::Utc::now().timestamp(), 0);
	sqlx::query!("INSERT INTO system (id, total_memory, used_memory, total_swap, used_swap, timestamp) VALUES ($1, $2, $3, $4, $5, $6)", sysid, sys.total_memory() as i64, sys.used_memory() as i64, sys.total_swap() as i64, sys.used_swap() as i64, now)
		.execute(&pool).await?;
	for (pid, process) in sys.processes() {
		let sql_pid = uuid::Uuid::new_v4();

		sqlx::query!("INSERT INTO process VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)", sql_pid, process.gid as i64, process.uid as i64, pid, process.name(), process.exe().to_str().unwrap(), BigDecimal::from_f32(process.cpu_usage()).unwrap(), process.memory() as i64, process.virtual_memory() as i64, process.start_time() as i64, process.parent(), sysid)
			.execute(&pool).await?;
		for (_, task) in process.tasks.iter() {
			let sql_tid = uuid::Uuid::new_v4();

			sqlx::query!("INSERT INTO thread VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)", sql_tid, task.gid as i64, task.uid as i64, BigDecimal::from_f32(task.cpu_usage()).unwrap(), task.memory() as i64, task.virtual_memory() as i64, task.start_time() as i64, task.parent(), sql_pid)
				.execute(&pool).await?;
		}
	}

	Ok(())
}
