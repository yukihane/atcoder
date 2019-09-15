import java.util.Scanner;

public class Main {

    public static void main(final String... args) {
        final String mode = "b";

        if (mode.equals("a")) {
            A.solve();
        } else if (mode.equals("b")) {
            B.solve();
        }
    }
}

class B {
    static void solve() {
        final Scanner sc = new Scanner(System.in);
        final int n = sc.nextInt();
        final int d = sc.nextInt();
        final int res = calc(n, d);
        System.out.println(res);
    }
    static int calc(final int n, final int d) {
        final int max = (2 * d + 1);
        int res = n / max;
        if (n % max != 0) {
            res += 1;
        }
        return res;
    }
}

class A {
    static void solve() {
        final Scanner sc = new Scanner(System.in);
        final int r = sc.nextInt();
        final int res = calc(r);
        System.out.println(res);
    }

    static int calc(final int r) {
        return 3 * r * r;
    }
}