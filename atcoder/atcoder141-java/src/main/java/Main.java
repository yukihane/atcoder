import java.util.Scanner;

public class Main {

    public static void main(final String... args) {
        final String mode = "b";

        if (mode.equals("a")) {
            A.solve();
        } else if (mode.equals("b")) {
            B.solve();
        } else if (mode.contentEquals("c")) {
            C.solve();
        }
    }
}

class B {
    static void solve() {
        final Scanner sc = new Scanner(System.in);
        final String str = sc.nextLine();
        final char[] s = str.toCharArray();
        final boolean res = calc(s);
        if (res) {
            System.out.println("Yes");
        } else {
            System.out.println("No");
        }
    }

    static boolean calc(final char[] s) {
        if (cond1(s) && cond2(s)) {
            return true;
        }
        return false;
    }

    private static boolean cond1(final char[] s) {
        for (int i = 0; i < s.length; i += 2) {
            if (s[i] == 'L') {
                return false;
            }
        }
        return true;
    }

    private static boolean cond2(final char[] s) {
        for (int i = 1; i < s.length; i += 2) {
            if (s[i] == 'R') {
                return false;
            }
        }
        return true;
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
