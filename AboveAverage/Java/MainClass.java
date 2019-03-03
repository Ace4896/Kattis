import java.util.Scanner;

public class MainClass {
  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);
    int testCases = Integer.parseInt(sc.nextLine());
    
    for (int i = 0; i < testCases; i++) {
      // Obtain input first
      String input = sc.nextLine();
      String[] splitInput = input.split(" ");
      int[] inputNums = new int[splitInput.length];
      int total = 0;
      
      // Cast to ints and add to total at same time
      for (int j = 0; j < splitInput.length; j++) {
        inputNums[j] = Integer.parseInt(splitInput[j]);
        total += inputNums[j];
      }
      
      // Subtract the extra one at the start (which indicates how many results there are)
      total -= inputNums[0];
      
      // Calculate average
      double average = (double)total / (double)inputNums[0];
      
      // Count how many are above this average
      double numAboveAverage = 0.0f;
      for (int k = 1; k < inputNums.length; k++) {
        if ((double)inputNums[k] > average) {
          numAboveAverage++;
        }
      }
      
      System.out.printf("%.3f%%\n", numAboveAverage / (double)inputNums[0] * 100);
    }
  }
}
