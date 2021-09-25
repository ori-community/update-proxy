class CacheItem {
  constructor(expiresAt, data) {
    this.expiresAt = expiresAt
    this.data = data
  }
}

export class Cache {
  constructor() {
    this.items = {}
  }

  async retrieve(key, ttl, getterPromise) {
    const existingItem = this.items[key]

    if (!existingItem || existingItem.expiresAt <= Date.now()) {
      this.items[key] = new CacheItem(Date.now() + ttl, await getterPromise())
    }

    return this.items[key].data
  }
}
