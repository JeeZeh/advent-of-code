import java.util.ArrayList;
import java.util.HashSet;
import java.util.Objects;

public class MultiState {
    String c1, c2, c3, c4;
    HashSet<String> have;

    public MultiState(String c1, String c2, String c3, String c4, HashSet<String> have) {
        this.c1 = c1;
        this.have = new HashSet<>(have);
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        MultiState that = (MultiState) o;
        return Objects.equals(c1, that.c1) && Objects.equals(c2, that.c2) && Objects.equals(c3, that.c3) && Objects.equals(c4, that.c4) && Objects.equals(have, that.have);
    }

    @Override
    public int hashCode() {
        return Objects.hash(c1, c2, c3, c4, have);
    }
}
