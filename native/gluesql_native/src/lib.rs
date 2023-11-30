use gluesql::prelude::{Glue, Payload, SharedMemoryStorage, Value};
use rustler::JobSpawner;
use rustler::{
    env::SavedTerm, Encoder, Env, LocalPid, NifStruct, OwnedBinary, OwnedEnv, ResourceArc, Term,
    ThreadSpawner,
};
use rustler_bigint::num_bigint::BigInt as NumBigInt;
use rustler_bigint::BigInt;
use std::{ops::Deref, panic::AssertUnwindSafe};

pub mod task;

#[derive(NifStruct)]
#[module = "Gluesql.MemoryDb"]
pub struct MemoryDb {
    storage: ResourceArc<MemoryStorageWrapper>,
}

pub struct MemoryStorageWrapper {
    inner: SharedMemoryStorage,
}

#[rustler::nif]
fn new_memory_db() -> MemoryDb {
    let storage = SharedMemoryStorage::new();

    MemoryDb {
        storage: ResourceArc::new(MemoryStorageWrapper { inner: storage }),
    }
}

#[rustler::nif]
fn execute_memory_db<'a>(
    _env: Env<'a>,
    db: MemoryDb,
    stmt: String,
    send_to: LocalPid,
) -> Result<(), String> {
    let xp = AssertUnwindSafe(db.storage.clone());

    ThreadSpawner::spawn(move || {
        task::block_on(async move {
            let asdf: SharedMemoryStorage = xp.inner.clone();
            let mut glue = Glue::new(asdf);
            let db_result = glue.execute(stmt).await;

            OwnedEnv::new().send_and_clear(&send_to, |thread_env| {
                match db_result {
                    Ok(result_set) => {
                        let formatted: Vec<_> = result_set
                            .into_iter()
                            .map(|res| payload_to_term(thread_env, res))
                            .collect();
                        let message = formatted.encode(thread_env);
                        message
                    }
                    Err(e) => {
                        let message = thread_env.error_tuple(format!("{:?}", e));
                        message
                    }
                }
            });
        })
    });

    Ok(())
}

fn payload_to_term<'a>(env: Env<'a>, payload: Payload) -> Term<'a> {
    match payload {
        Payload::Select { labels, rows } => {
            let mut output = Vec::with_capacity(rows.len());
            let keys: Vec<_> = labels.into_iter().map(|key| key.encode(env)).collect();

            for row in rows {
                let values: Vec<_> = row.into_iter().map(|val| value_to_term(env, val)).collect();
                let map = Term::map_from_arrays(env, &keys, &values)
                    .expect("keys and values are not the same length");
                output.push(map);
            }

            output.encode(env)
        }
        _ => ().encode(env),
    }
}

fn value_to_term<'a>(env: Env<'a>, value: Value) -> Term<'a> {
    match value {
        Value::Bool(value) => value.encode(env),
        Value::I8(value) => value.encode(env),
        Value::I16(value) => value.encode(env),
        Value::I32(value) => value.encode(env),
        Value::I64(value) => value.encode(env),
        Value::I128(value) => BigInt::from(NumBigInt::from(value)).encode(env),
        Value::U8(value) => value.encode(env),
        Value::U16(value) => value.encode(env),
        Value::U32(value) => value.encode(env),
        Value::U64(value) => value.encode(env),
        Value::U128(value) => BigInt::from(NumBigInt::from(value)).encode(env),
        Value::F32(value) => value.encode(env),
        Value::F64(value) => value.encode(env),
        Value::Decimal(value) => value.to_string().encode(env),
        Value::Str(value) => value.encode(env),
        Value::Bytea(value) => value.encode(env),
        Value::Inet(value) => value.to_string().encode(env),
        Value::Date(value) => value.to_string().encode(env),
        Value::Timestamp(value) => value.to_string().encode(env),
        Value::Time(value) => value.to_string().encode(env),
        Value::Interval(value) => format!("{:?}", value).encode(env),
        Value::Uuid(value) => value.to_string().encode(env),
        Value::Map(value) => format!("{:?}", value).encode(env),
        Value::List(value) => format!("{:?}", value).encode(env),
        Value::Point(value) => value.to_string().encode(env),
        Value::Null => rustler::types::atom::nil().encode(env),
    }
}

pub fn on_load(env: Env, _: rustler::Term) -> bool {
    rustler::resource!(MemoryStorageWrapper, env);
    true
}

rustler::init!(
    "Elixir.Gluesql.Native",
    [new_memory_db, execute_memory_db],
    load = on_load
);
