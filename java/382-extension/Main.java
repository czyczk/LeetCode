public class Main {
    public static void main(String[] args) {
        var temp = new ListNode(9);
		var head = new ListNode(8);
		head.next = temp;
		temp = head;
        head = new ListNode(7);
		head.next = temp;
		temp = head;
        head = new ListNode(6);
		head.next = temp;
		temp = head;
        head = new ListNode(5);
		head.next = temp;
		temp = head;
        head = new ListNode(4);
		head.next = temp;
		temp = head;
        head = new ListNode(3);
		head.next = temp;
		temp = head;
        head = new ListNode(2);
		head.next = temp;
		temp = head;
        head = new ListNode(1);
		head.next = temp;

        Solution obj = new Solution(head);
        int k = 4;
        var param_1 = obj.getRandomK(k);

        System.out.println("List length: " + param_1.size());
        for (var num : param_1) {
            System.out.println(num);
        }
    }
}
