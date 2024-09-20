use tus_yuurikai_system::infrastracture::router::App;

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
    match app.auth.auth_repository.delete_all().await {
        Ok(_) => {},
        Err(err) => panic!("{}", err),
    }
}