import java.util.HashMap;
import java.util.LinkedList;

public class Main {
	public static void main(final String[] args) {
		final var c = new LFUCache(2);
		c.put(1, 1);
		c.put(2, 2);
		// Expecting 1
		System.out.println(c.get(1));
		c.put(3, 3);
		// Expecting -1
		System.out.println(c.get(2));
		// Expecting 3
		System.out.println(c.get(3));
		c.put(4, 4);
		// Expecting -1
		System.out.println(c.get(1));
		// Expecting 3
		System.out.println(c.get(3));
		// Expecting 4
		System.out.println(c.get(4));
	}
}

class LFUCache {
	private final int capacity;
	private final HashMap<Integer, CacheItem> cacheMap;
	private final HashMap<Integer, LinkedList<CacheItem>> freqMap;
	private int minFreq;

	public LFUCache(final int capacity) {
		this.capacity = capacity;
		this.cacheMap = new HashMap<>();
		this.freqMap = new HashMap<>();
		this.minFreq = 1;
	}

	public int get(final int key) {
		if (this.capacity == 0) {
			return -1;
		}

		final var item = this.cacheMap.get(key);
		if (item == null) {
			return -1;
		}

		// Update the freq: freq += 1
		final var oldFreq = item.getFreq();
		item.incrFreq();

		// Remove the CacheItem from the linked list of oldFreq and add it to the
		// linked list of the new freq.
		final var oldFreqList = this.freqMap.get(oldFreq);
		oldFreqList.removeFirstOccurrence(item);
		if (oldFreq == this.minFreq && oldFreqList.isEmpty()) {
			// If the linked list of oldFreq is empty and has an impact on minFreq,
			// increment minFreq by 1, as minReq is now the updated freq.
			this.minFreq++;
		}

		var newFreqList = this.freqMap.get(item.getFreq());
		if (newFreqList == null) {
			newFreqList = new LinkedList<>();
			this.freqMap.put(item.getFreq(), newFreqList);
		}
		newFreqList.offer(item);

		return item.getValue();
	}

	public void put(final int key, final int value) {
		if (this.capacity == 0) {
			return;
		}

		final var existingVal = this.get(key);
		if (existingVal == -1) {
			if (this.cacheMap.size() == this.capacity) {
				// The cache is full, remove the LFU (or LRU on tie) item.
				// No need to consider if the freq list is empty, since a new item
				// will be inserted soon, which will make minFreq 1 again.
				final var lfuItem = this.freqMap.get(this.minFreq).poll();
				this.cacheMap.remove(lfuItem.getKey());
			}

			final var newItem = new CacheItem(key, value);

			this.cacheMap.put(key, newItem);

			var freqList = this.freqMap.get(1);
			if (freqList == null) {
				freqList = new LinkedList<>();
				this.freqMap.put(1, freqList);
			}
			freqList.offer(newItem);

			this.minFreq = 1;
		} else if (existingVal != value) {
			this.cacheMap.get(key).setValue(value);
		}
	}
}

class CacheItem {
	private int key;
	private int value;
	private int freq;

	public CacheItem(final int key, final int value) {
		this.key = key;
		this.value = value;
		this.freq = 1;
	}

	public int getKey() {
		return key;
	}

	public void setKey(final int key) {
		this.key = key;
	}

	public int getValue() {
		return value;
	}

	public void setValue(final int value) {
		this.value = value;
	}

	public int getFreq() {
		return freq;
	}

	public void incrFreq() {
		this.freq++;
	}

}
