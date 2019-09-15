import java.util.Scanner;

public class Main {

    public static void main(final String... args) {
        final String mode = "c";

        if (mode.equals("a")) {
            A.solve();
        } else if (mode.equals("b")) {
            B.solve();
        } else if (mode.contentEquals("c")) {
            C.solve();
        }
    }
}

class C {

    static void solve() {
        final Scanner sc = new Scanner(System.in);
        final int n = sc.nextInt();
        final int k = sc.nextInt();
        final int q = sc.nextInt();
        final int[] a = new int[q];
        for (int i = 0; i < q; i++) {
            a[i] = sc.nextInt();
        }
        final int[] res = calc(n, k, q, a);
        for (final int p : res) {
            if (q - p < k) {
                System.out.println("Yes");
            } else {
                System.out.println("No");
            }
        }
    }

    static int[] calc(final int n, final int k, final int q, final int[] correctors) {
        final int[] points = new int[n];
        for (final int col : correctors) {
            points[col - 1]++;
        }
        return points;
     }
}

class B {
    static void solve() {
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