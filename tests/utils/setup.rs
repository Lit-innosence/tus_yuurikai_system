use tus_yuurikai_system::infrastructure::router::App;

pub async fn setup_db(app: &App) {
    match app.assignment_record.assignment_record_repository.delete_all().await {
        Ok(_) => {},
        Err(err) => panic!("{}", err),
    }
    match app.student_pair.student_pair_repository.delete_all().await {
        Ok(_) => {},
        Err(err) => panic!("{}", err),
    }
    match app.student.student_repository.delete_all().await {
        Ok(_) => {},
        Err(err) => panic!("{}", err),
    }
    match app.auth.locker_auth_info_repository.delete_all().await {
        Ok(_) => {},
        Err(err) => panic!("{}", err),
    }
    match app.auth.auth_repository.delete_all().await {
        Ok(_) => {},
        Err(err) => panic!("{}", err),
    }
    if app.locker.locker_repository.update_all_status(&String::from("vacant")).await.is_err() {
        panic!("failed to update locker status");
    }
}