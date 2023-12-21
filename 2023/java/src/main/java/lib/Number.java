package lib;

public class Number {

  public static long gcd(long a, long b) {
    if (b == 0) {
      return a;
    }
    return gcd(b, a % b);
  }

  public static long lcm(long a, long b) {
    if (a > b) {
      return (a / gcd(a, b)) * b;
    } else {
      return (b / gcd(a, b)) * a;
    }
  }

  public static long mod(long lhs, long rhs) {
    var res = lhs % rhs;
    if (res < 0) {
      return res + rhs;
    }
    return res;
  }
}
