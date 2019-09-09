import java.util.Scanner;

public class Main {
    public static void main(final String[] args) {
        final int p = 5;
        switch (p) {
        case 6:
        case 5:
            E.answer();
            break;
        case 4:
            D.answer();
            break;
        case 3:
            C.answer();
            break;
        case 2:
            B.answer();
            break;
        case 1:
            A.answer();
            break;
        }
    }
}

class E {

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
        int total = 0;
        final int prev[] = new int[] { -1, -1 };
        for (int i = 0; i < array.length - 1; i++) {
            final int largest[] = new int[] { array[i], -1 };
            int subTotal = 0;
            for (int j = i + 1; j < array.length; j++) {
                final int num = array[j];
                if (num != largest[0] && num != largest[1]) {
                    if (num > largest[0]) {
                        largest[1] = largest[0];
                        largest[0] = num;
                    } else if (num > largest[1]) {
                        largest[1] = num;
                    }
                }
//                System.out.println("" + i + "," + j + "," + num + "," + largest[0] + "," + largest[1]);
                subTotal += largest[1];
            }
            total += subTotal;
            prev[0] = largest[0];
            prev[1] = largest[1];
        }
        return total;
    }

    private int second(final int[] subArray) {
        // TODO Auto-generated method stub
        return 0;
    }

    private int[] makeSubArray(final int[] array, final int i, final int j) {
        // TODO Auto-generated method stub
        return null;
    }
}

class D {

    static String drawUp(final String input, final int l, final int r) {
        final char[] chars = input.toCharArray();
        for (int i = l; i < r; i++) {
            final char left = chars[i];
            final char right = chars[r - i];
            chars[i] = reverse(right);
            chars[r - i] = reverse(left);
        }
        return String.copyValueOf(chars);
    }

    static char reverse(final char c) {
        switch (c) {
        case 'L':
            return 'R';
        case 'R':
        default:
            return 'L';
        }
    }

    static void answer() {
        final Scanner sc = new Scanner(System.in);
        final int n = sc.nextInt();
        final int[] array = new int[n];
        array[0] = Integer.MAX_VALUE;
        for (int i = 1; i < n; i++) {
            array[i] = sc.nextInt();
        }
//
//        final int res = calc(array);
//        System.out.println(res);
    }
}

class C {

    static void answer() {
        final Scanner sc = new Scanner(System.in);
        final int n = sc.nextInt();
        final int[] array = new int[n];
        array[0] = Integer.MAX_VALUE;
        for (int i = 1; i < n; i++) {
            array[i] = sc.nextInt();
        }

        final int res = calc(array);
        System.out.println(res);
    }

    static int calc(final int[] array) {
        int total = 0;
        for (int i = 0; i < array.length - 1; i++) {
            total += Math.min(array[i], array[i + 1]);
        }
        total += array[array.length - 1];
        return total;
    }
}

class B {

    static void answer() {
        final Scanner sc = new Scanner(System.in);
        final int n = sc.nextInt();
        final int[] arrayA = new int[n];
        for (int i = 0; i < n; i++) {
            arrayA[i] = sc.nextInt();
        }
        final int[] arrayB = new int[n];
        for (int i = 0; i < n; i++) {
            arrayB[i] = sc.nextInt();
        }
        final int[] arrayC = new int[n];
        for (int i = 0; i < n - 1; i++) {
            arrayC[i] = sc.nextInt();
        }
        final int res = calc(arrayA, arrayB, arrayC);
        System.out.println(res);
    }

    static int calc(final int[] arrayA, final int[] arrayB, final int[] arrayC) {
        int total = 0;
        int prevRyori = -100;
        for (int i = 0; i < arrayA.length; i++) {
            final int ryoriNo = arrayA[i];
            final int manpuku = arrayB[ryoriNo - 1];
            total += manpuku;
            if (ryoriNo - prevRyori == 1) {
                total += arrayC[prevRyori - 1];
            }
            prevRyori = ryoriNo;
        }

        return total;
    }
}

class A {
    static void answer() {
        final Scanner sc = new Scanner(System.in);
        final int n = sc.nextInt();
        final int res = calc(n);
        System.out.println(res);
    }

    static int calc(final int n) {
        return n * n * n;
    }
}