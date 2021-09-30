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
      const promise = new Promise(async (resolve, reject) => {
        try {
          resolve(await getterPromise)
        } catch (e) {
          reject(e)
        }
      })
        .then(value => {
          this.items[key] = new CacheItem(Date.now() + ttl, value)
        })
        .catch(console.error)

      if (!existingItem) {
        await promise
      }
    }

    return this.items[key].data
  }
}
