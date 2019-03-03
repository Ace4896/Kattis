import java.util.Scanner;

public class MainClass {
  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);
    int testCases = sc.nextInt();
    
    for (int i = 0; i < testCases; i++) {
      long leftMoves = sc.nextLong();
      long rightMoves = sc.nextLong();
      long n = leftMoves + rightMoves;
      
      System.out.printf("%d\n", (((n * n) + n + 2) / 2) + rightMoves);
    }
  }
}
