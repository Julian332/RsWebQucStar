use std::sync::OnceLock;
use std::time::Duration;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

#[tokio::main]
async fn main() -> Result<(), JobSchedulerError> {
  let mut sched = JobScheduler::new().await?;
  // Add basic cron job
  sched.add(
    Job::new("1/10 * * * * *", |_uuid, _l| {
      println!("I run every 10 seconds");
    })?
  ).await?;

  // Add async job
  sched.add(
    Job::new_async("1/7 * * * * *", |uuid, mut l| {
      Box::pin(async move {
        println!("I run async every 7 seconds");

        // Query the next execution time for this job
        let next_tick = l.next_tick_for_job(uuid).await;
        match next_tick {
          Ok(Some(ts)) => println!("Next time for 7s job is {:?}", ts),
          _ => println!("Could not get next tick for 7s job"),
        }
      })
    })?
  ).await?;

  // Needs the `english` feature enabled
  sched.add(
    Job::new_async("every 4 seconds", |uuid, mut l| {
      Box::pin(async move {
        println!("I run async every 4 seconds");

        // Query the next execution time for this job
        let next_tick = l.next_tick_for_job(uuid).await;
        match next_tick {
          Ok(Some(ts)) => println!("Next time for 4s job is {:?}", ts),
          _ => println!("Could not get next tick for 4s job"),
        }
      })
    })?
  ).await?;

  // Add one-shot job with given duration
  sched.add(
    Job::new_one_shot(Duration::from_secs(18), |_uuid, _l| {
      println!("I only run once");
    })?
  ).await?;

  // Create repeated job with given duration, make it mutable to edit it afterwards
  let mut jj = Job::new_repeated(Duration::from_secs(8), |_uuid, _l| {
    println!("I run repeatedly every 8 seconds");
  })?;

  // Add actions to be executed when the jobs starts/stop etc.
  jj.on_start_notification_add(&sched, Box::new(|job_id, notification_id, type_of_notification| {
    Box::pin(async move {
      println!("Job {:?} was started, notification {:?} ran ({:?})", job_id, notification_id, type_of_notification);
    })
  })).await?;

  jj.on_stop_notification_add(&sched, Box::new(|job_id, notification_id, type_of_notification| {
    Box::pin(async move {
      println!("Job {:?} was completed, notification {:?} ran ({:?})", job_id, notification_id, type_of_notification);
    })
  })).await?;

  jj.on_removed_notification_add(&sched, Box::new(|job_id, notification_id, type_of_notification| {
    Box::pin(async move {
      println!("Job {:?} was removed, notification {:?} ran ({:?})", job_id, notification_id, type_of_notification);
    })
  })).await?;
  sched.add(jj).await?;

  // Feature 'signal' must be enabled
  // sched.shutdown_on_ctrl_c();

  // Add code to be run during/after shutdown
  sched.set_shutdown_handler(Box::new(|| {
    Box::pin(async move {
      println!("Shut down done");
    })
  }));

  // Start the scheduler
  sched.start().await?;

  // Wait while the jobs run
  tokio::time::sleep(Duration::from_secs(100)).await;

  Ok(())
}
