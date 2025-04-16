use tus_yuurikai_system::infrastructure::router::App;
use rocket::tokio::task;

pub async fn setup_db(app: &App) {
    let assignment_record_repository = app.assignment_record.assignment_record_repository.clone();
    match task::spawn_blocking(move || {
        assignment_record_repository.delete_all()
    }).await {
        Ok(Ok(_)) => {},
        Ok(Err(err)) => panic!("{}", err),
        Err(err) => panic!("{}", err),
    }

    let student_pair_repository = app.student_pair.student_pair_repository.clone();
    match task::spawn_blocking(move || {
        student_pair_repository.delete_all()
    }).await {
        Ok(Ok(_)) => {},
        Ok(Err(err)) => panic!("{}", err),
        Err(err) => panic!("{}", err),
    }

    let student_repository = app.student.student_repository.clone();
    match task::spawn_blocking(move || {
        student_repository.delete_all()
    }).await {
        Ok(Ok(_)) => {},
        Ok(Err(err)) => panic!("{}", err),
        Err(err) => panic!("{}", err),
    }

    let locker_auth_info_repository = app.auth.locker_auth_info_repository.clone();
    match task::spawn_blocking(move || {
        locker_auth_info_repository.delete_all()
    }).await {
        Ok(Ok(_)) => {},
        Ok(Err(err)) => panic!("{}", err),
        Err(err) => panic!("{}", err),
    }

    let circle_auth_info_repository = app.auth.circle_auth_info_repository.clone();
    match task::spawn_blocking(move || {
        circle_auth_info_repository.delete_all()
    }).await {
        Ok(Ok(_)) => {},
        Ok(Err(err)) => panic!("{}", err),
        Err(err) => panic!("{}", err),
    }

    let auth_repository = app.auth.auth_repository.clone();
    match task::spawn_blocking(move || {
        auth_repository.delete_all()
    }).await {
        Ok(Ok(_)) => {},
        Ok(Err(err)) => panic!("{}", err),
        Err(err) => panic!("{}", err),
    }

    let locker_repository = app.locker.locker_repository.clone();
    match task::spawn_blocking(move || {
        locker_repository.update_all_status(String::from("vacant"))
    }).await {
        Ok(Ok(_)) => {},
        _ => panic!("failed to update locker status"),
    }
}