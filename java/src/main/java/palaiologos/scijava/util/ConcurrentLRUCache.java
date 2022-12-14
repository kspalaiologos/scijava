package palaiologos.scijava.util;

import java.util.concurrent.locks.ReentrantLock;

public class ConcurrentLRUCache<K, V> {
    public V get(K key) {
        lock.lock();
        V x = null;
        try {
            x = cache.get(key);
        } catch(Throwable t) {
            lock.unlock();
            throw t;
        }
        lock.unlock();
        return x;
    }

    public void put(K key, V value) {
        lock.lock();
        try {
            cache.put(key, value);
        } catch(Throwable t) {
            lock.unlock();
            throw t;
        }
        lock.unlock();
    }

    public ConcurrentLRUCache(int maxSize) {
        cache = new LRUCache<>(maxSize);
    }

    private final LRUCache<K, V> cache;
    private final ReentrantLock lock = new ReentrantLock();
}