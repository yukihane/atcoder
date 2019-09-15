import java.util.ArrayList;
import java.util.List;
import java.util.Scanner;

public class Main {

    public static void main(final String... args) {
        final String mode = "d";

        if (mode.equals("a")) {
            A.solve();
        } else if (mode.equals("b")) {
            B.solve();
        } else if (mode.contentEquals("c")) {
            C.solve();
        } else if (mode.contentEquals("d")) {
            D.solve();
        }
    }
}

class D {
    static void solve() {
        final Scanner sc = new Scanner(System.in);
        final int n = sc.nextInt();
        final int m = sc.nextInt();
        final List<Integer> a = new ArrayList<>(n);
        for (int i = 0; i < n; i++) {
            a.add(sc.nextInt());
        }
        final long res = calc(n, m, a);
        System.out.println(res);
    }

    static long calc(final int n, final int m, final List<Integer> a) {
        if (n == 0) {
            return 0L;
        }
        final int[] aArray = initialize(a);
        for (int i = 0; i < m; i++) {
//            System.out.println(Arrays.toString(aArray));
            aArray[0] /= 2;
            sort(aArray);
            if (aArray[0] == 0) {
                return 0;
            }
        }
        // 合計金額
        long total = 0L;
        for (int i = 0; i < n; i++) {
            total += aArray[i];
        }
//        System.out.println(total);
        return total;
    }

    static void sort(final int[] a) {
        // 挿入ソート
        final int point = hakken(a, a[0]);
        if (point > 0) {
            zurasu(a, point);
        }
    }

    private static int hakken(final int[] a, final int i) {
        // 2分探索
        final int min = 1;
        final int max = a.length - 1;
        return search(a, i, min, max);
    }

    private static int search(final int[] a, final int i, final int min, final int max) {
//        System.out.println("" + min + "," + max);
        final int cur = (min + max) / 2;
        if (cur == 0) {
            return 0;
        }
        if (a[cur] <= i && a[cur - 1] >= i) {
            return cur - 1;
        }
        if (min == max || min + 1 == max) {
            return 0;
        }
        if (i > a[cur]) {
            return search(a, i, min, max - (max - min) / 2);
        } else {
            return search(a, i, min + (max - min) / 2, max);
        }
    }

    private static void zurasu(final int[] a, final int index) {
        for (int i = 0; i < index; i++) {
            a[i] = a[i + 1];
        }
    }

    static int[] initialize(final List<Integer> a) {
        // 降順ソート
        a.sort((x, y) -> y - x);
        final int[] res = new int[a.size()];
        for (int i = 0; i < a.size(); i++) {
            res[i] = a.get(i);
        }
        return res;
    }
}

class A {
    static void solve() {
        final Scanner sc = new Scanner(System.in);
        final String s = sc.nextLine();
        final String res = calc(s);
        System.out.println(res);
    }

    static String calc(final String s) {
        switch (s) {
        case "Sunny":
            return "Cloudy";
        case "Cloudy":
            return "Rainy";
        case "Rainy":
            return "Sunny";
        }
        return null;
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
