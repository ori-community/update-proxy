class CacheItem {
  constructor(expiresAt, data) {
    this.expiresAt = expiresAt
    this.data = data
  }
}

export class LazyCache {
  constructor() {
    this.items = {}
  }

  retrieve(key, ttl, getterPromise) {
    const existingItem = this.items[key]

    if (!existingItem || existingItem.expiresAt <= Date.now()) {
      getterPromise
        .then(value => {
          this.items[key] = new CacheItem(Date.now() + ttl, value)
        })
        .catch(console.error)
    }

    return this.items[key].data
  }
}
