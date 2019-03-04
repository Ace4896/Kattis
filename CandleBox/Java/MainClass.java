import java.util.Scanner;

public class MainClass {
  public static void main(String[] args) {
    // Read in input
    Scanner sc = new Scanner(System.in);
    int D = sc.nextInt();
    int R = sc.nextInt();
    int T = sc.nextInt();

    // Determine Rita's age
    // Some variables for quadratic formula solving (see notes)
    int b = 1 - D;
    int c = (D * D) - D - 18 - (2 * (R + T));

    int n = (- b + (int)Math.sqrt((b * b) - (2 * c))) / 2;

    // Check if Theo's age is at least 3 years old (i.e. if he should have any candles)
    if (n - D < 3) {
      // If he is less than 3 years old, no candles should be transferred
      System.out.println(0);
    }
    else {
      // Otherwise, calculate the no. of candles that Rita should have
      // Then, obtain difference to get no. of candles to transfer over
      int expectedRitaCandles = ((n * n) + n - 12) / 2;
      System.out.println(R - expectedRitaCandles);
    }
  }
}
