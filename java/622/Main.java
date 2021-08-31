public class Main {
	public static void main(final String[] args) {
		final var q = new MyCircularQueue(3);

		// Expecting true
		System.out.println(q.isEmpty());
		// Expecting false
		System.out.println(q.isFull());
		// Expecting true
		System.out.println(q.enQueue(1));
		// Expecting true
		System.out.println(q.enQueue(2));
		// Expecting true
		System.out.println(q.enQueue(3));
		// Expecting true
		System.out.println(q.isFull());
		// Expecting false (is full)
		System.out.println(q.enQueue(4));
		// Expecting 1
		System.out.println(q.Front());
		// Expecting 3
		System.out.println(q.Rear());
		// Expecting true
		System.out.println(q.deQueue());
		// Expecting 2
		System.out.println(q.Front());
		// Expecting true
		System.out.println(q.enQueue(4));
		// Expecting 4
		System.out.println(q.Rear());
	}
}

class MyCircularQueue {
	private final int[] queue;
	private final int capacity;
	private int size;
	private int cur; // The rear idx

	public MyCircularQueue(final int k) {
		this.queue = new int[k];
		this.capacity = k;
	}

	public boolean enQueue(final int value) {
		if (this.isFull()) {
			return false;
		}

		this.queue[this.cur] = value;
		this.size++;
		this.cur = this.getNextCur();
		return true;
	}

	public boolean deQueue() {
		if (this.isEmpty()) {
			return false;
		}

		this.size--;
		return true;
	}

	public int Front() {
		if (this.isEmpty()) {
			return -1;
		}

		return this.queue[this.getFrontCur()];
	}

	public int Rear() {
		if (this.isEmpty()) {
			return -1;
		}

		return this.queue[this.getPreviousCur()];
	}

	public boolean isEmpty() {
		return this.size == 0;
	}

	public boolean isFull() {
		return this.size == this.capacity;
	}

	private int getPreviousCur() {
		return (this.cur + this.capacity - 1) % this.capacity;
	}

	private int getNextCur() {
		return (this.cur + 1) % this.capacity;
	}

	private int getFrontCur() {
		return (this.cur + this.capacity - this.size) % this.capacity;
	}
}
