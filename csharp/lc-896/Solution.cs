namespace lc_896
{
    public class Solution
    {
        public bool IsMonotonic(int[] a)
        {
            bool? trend = null;
            var n = a.Length;
            if (n == 1)
            {
                return true;
            }

            for (var i = 1; i < n; i++)
            {
                var cur = a[i];
                var last = a[i - 1];
                if (cur > last)
                {
                    if (trend == false)
                    {
                        return false;
                    }
                    else
                    {
                        trend = true;
                    }
                }
                else if (cur < last)
                {
                    if (trend == true)
                    {
                        return false;
                    }
                    else
                    {
                        trend = false;
                    }
                }
            }

            return true;
        }
    }
}
