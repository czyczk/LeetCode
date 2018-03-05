import java.math.BigDecimal
import java.util.*
import kotlin.collections.HashMap

fun main(args: Array<String>) {
    val solution = Solution()
    var l1: ListNode? = null
    var l2: ListNode? = null
//    l1 = solution.addToPrev(3, l1)
//    l1 = solution.addToPrev(4, l1)
//    l1 = solution.addToPrev(2, l1)
//    l2 = solution.addToPrev(4, l2)
//    l2 = solution.addToPrev(6, l2)
//    l2 = solution.addToPrev(5, l2)

    l1 = solution.addToPrev(8, l1)
    l1 = solution.addToPrev(9, l1)
    l2 = solution.addToPrev(1, l2)
    var result: ListNode? = Solution().addTwoNumbers(l1, l2)
    while (result != null) {
        println(result.`val`)
        result = result.next
    }

}

/**
 * Definition for singly-linked list.
 * class ListNode(var `val`: Int = 0) {
 *     var next: ListNode? = null
 * }
 */
class Solution {
    fun addTwoNumbers(l1: ListNode, l2: ListNode): ListNode {
        var curL1: ListNode? = l1
        var curL2: ListNode? = l2
        var result: ListNode? = null
        var firstNode: ListNode? = null
        var carry = 0
        while (curL1 != null || curL2 != null) {
            var sum = 0
            if (curL1 != null) {
                sum = curL1.`val`
                curL1 = curL1.next
            }
            if (curL2 != null) {
                sum += curL2.`val`
                curL2 = curL2.next
            }
            sum += carry
            carry = sum / 10
            result = addToLast(sum % 10, result)
            if (firstNode == null)
                firstNode = result
        }
        if (carry > 0)
            result = addToLast(carry, result)
        return firstNode!!
    }

    fun addToLast(element: Int, listNode: ListNode?): ListNode {
        val newNode = ListNode(element)
        if (listNode != null)
            listNode.next = newNode
        return newNode
    }

    fun addToPrev(element: Int, listNode: ListNode?): ListNode {
        val newNode = ListNode(element)
        newNode.next = listNode
        return newNode
    }

    private fun length(listNode: ListNode): Int {
        var len = 0
        var curNode: ListNode? = listNode
        while (curNode != null) {
            len++
            curNode = curNode.next
        }
        return len
    }
}

data class ListNode(var `val`: Int = 0) {
    var next: ListNode? = null
}