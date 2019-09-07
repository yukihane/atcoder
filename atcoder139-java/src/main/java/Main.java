import java.util.Objects;
import java.util.Scanner;

public class Main {
    public static void main(final String[] args) {
        final int p = 4;
        switch (p) {
        case 4:
            P4.answer();
        case 3:
            P3.answer();
            break;
        case 2:
            P2.answer();
            break;
        case 1:
            P1.p1();
            break;
        }
    }
}

class P4 {
    static void answer() {
        final Scanner sc = new Scanner(System.in);
        final int n = sc.nextInt();
        final int[] array = new int[n];
        for (int i = 0; i < n; i++) {
            array[i] = sc.nextInt();
        }
        final int res = calc(array);
        System.out.println(res);
    }

}

class P3 {
    static void answer() {
        final Scanner sc = new Scanner(System.in);
        final int n = sc.nextInt();
        final int[] array = new int[n];
        for (int i = 0; i < n; i++) {
            array[i] = sc.nextInt();
        }
        final int res = calc(array);
        System.out.println(res);
    }

    static int calc(final int[] array) {
        int max = 0;
        for (int i = 0; i < array.length; i++) {
            if (max > array.length - i) {
                break;
            }
            final int res = measure(array, i);
            max = Math.max(res, max);
        }
        return max;
    }

    private static int measure(final int[] array, final int pos) {
        int times = 0;
        for (int i = pos; i < array.length - 1; i++) {
            if (array[i] >= array[i + 1]) {
                times++;
            } else {
                break;
            }
        }
        return times;
    }

}

class P2 {
    static void answer() {
        final Scanner sc = new Scanner(System.in);
        final int a = sc.nextInt();
        final int b = sc.nextInt();
        final int res = calc(a, b);
        System.out.println(res);
    }

    static int calc(final int a, final int b) {
        final int per = a - 1;
        final int remain = b - 1;
        final int x = remain / per;
        if (remain - (per * x) > 0) {
            return x + 1;
        }
        return x;
    }
}

class P1 {

    static void p1() {
        final Scanner sc = new Scanner(System.in);
        final String yohou = sc.nextLine();
        final String jissai = sc.nextLine();

        final int res = calc(yohou, jissai);
        System.out.println(res);
    }

    static int calc(final String yohou, final String kekka) {
        int res = 0;
        for (int i = 0; i < 3; i++) {
            if (Objects.equals(yohou.charAt(i), kekka.charAt(i))) {
                res++;
            }
        }
        return res;
    }
}