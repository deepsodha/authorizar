use crate::routes::app_state::AppState;
use crate::routes::{entities_config, policies_config};
use actix_cors::Cors;
use actix_web::{http::header, middleware, middleware::Logger, web, App, HttpServer};
use cedar_authorizer::http::authz::authorize;
use cedar_authorizer::routes::api_error::ApiError;
use cedar_authorizer::routes::health_check;
use cedar_authorizer::utils::env_helper::AppEnv;
use dotenv::var;
use sqlx::migrate::MigrateDatabase;
use sqlx::migrate::Migrator;
use sqlx::Sqlite;
use std::path::Path;

pub async fn server() -> Result<(), ApiError> {
    let app_environment = AppEnv::current_env()?;
    let dev_db = var("DB_FILE").expect("DB name must be set");
    let url = var("URL").expect("url must be set in env");
    let db_conn = format!("sqlite://{}", &dev_db);

    if !Sqlite::database_exists(&db_conn).await.unwrap_or(false) {
        println!("Creating database {}", db_conn);
        match Sqlite::create_database(&db_conn).await {
            Ok(_) => println!("Create db success for {}", db_conn),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("{} Database already exists", db_conn);
    }

    let pool = sqlx::sqlite::SqlitePool::connect(&format!("sqlite:{}", &dev_db))
        .await
        .expect("Failed to connect to the database");

    let migrator = Migrator::new(Path::new("././migrations")).await.unwrap();
    migrator
        .run(&pool)
        .await
        .expect("Failed to migrate the database");
    let app_state = AppState { pool };
    let server = HttpServer::new(move || {
        let cors_base = Cors::default()
            .allowed_methods(vec!["POST", "GET"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);

        let cors = match app_environment {
            AppEnv::Development => cors_base
                .allowed_origin("http://localhost:5000")
                .allowed_origin("http://localhost:8080"),
            AppEnv::Production => cors_base
                .allowed_origin("http://localhost:5000")
                .allowed_origin("http://localhost:8080"),
            AppEnv::Staging => cors_base.allowed_origin("http://localhost:8080"),
            AppEnv::Test => cors_base,
        };

        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(Logger::new("%a %{User-Agent}i - %D millisecond"))
            .wrap(middleware::Compress::default())
            .wrap(cors)
            .service(
                web::scope("/api")
                    .service(web::scope("/entities").configure(entities_config))
                    .service(web::scope("/policies").configure(policies_config))
                    .route("/authorize", web::post().to(authorize)),
            )
            .route("/health_check", web::get().to(health_check))
    });
    let _res = server.bind(&url)?.run().await;
    Ok(())
}
