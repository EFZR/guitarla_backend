// region:     --- Modules

mod dev_db;

use crate::ctx::Ctx;
use crate::model::guitars::{Guitarra, GuitarraBmc, GuitarraForCreate};
use crate::model::{self, ModelManager};
use tokio::sync::OnceCell;
use tracing::info;

// endregion:  --- Modules

/// Initialize environment for local developement.
/// (for early developement, will be called from main)
pub async fn init_dev() {
    static INIT: OnceCell<()> = OnceCell::const_new();

    INIT.get_or_init(|| async {
        info!("{:<12} - init_dev_all()", "FOR-DEV-ONLY");

        dev_db::init_dev_db().await.unwrap();
    })
    .await;
}

/// Initialize test envioronment.
pub async fn init_test() -> ModelManager {
    static INIT: OnceCell<ModelManager> = OnceCell::const_new();

    let mm = INIT
        .get_or_init(|| async {
            init_dev().await;
            ModelManager::new().await.unwrap()
        })
        .await;

    mm.clone()
}

pub async fn seed_guitars(
    ctx: &Ctx,
    mm: &ModelManager,
    guitars: &[&str],
) -> model::Result<Vec<Guitarra>> {
    let mut guitarras = Vec::new();

    for guitar in guitars {
        let id = GuitarraBmc::create(
            ctx,
            mm,
            GuitarraForCreate {
                name: guitar.to_string(),
                description: "dummy description".to_string(),
                price: 0.00,
                img: None,
            },
        )
        .await?;

        let item = GuitarraBmc::get(ctx, mm, id).await?;

        guitarras.push(item);
    }

    Ok(guitarras)
}
