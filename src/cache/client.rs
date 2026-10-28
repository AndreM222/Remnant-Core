use redis_pool::RedisPool;

pub async fn connect(
    url: &str,
) -> anyhow::Result<RedisPool<redis::Client, redis::aio::MultiplexedConnection>> {
    let client = redis::Client::open(url)?;
    let pool = RedisPool::from(client);
    Ok(pool)
}
