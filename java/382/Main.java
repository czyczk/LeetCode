public class Main {
    public static void main(String[] args) {
        var temp = new ListNode(4);
		var head = new ListNode(3);
		head.next = temp;
		temp = head;
        head = new ListNode(2);
		head.next = temp;
		temp = head;
        head = new ListNode(1);
		head.next = temp;

        Solution obj = new Solution(head);
        int param_1 = obj.getRandom();

        System.out.println(param_1);
    }
}
