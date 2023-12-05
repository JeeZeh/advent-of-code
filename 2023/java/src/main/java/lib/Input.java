package lib;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.stream.Stream;

public class Input {

  public static Stream<String> lines(String fileName) throws IOException {
    return Files.lines(Paths.get("src/main/java/" + fileName));
  }

  public static String read(String fileName) throws IOException {
    return Files.readString(Paths.get("src/main/java/" + fileName));
  }
}
