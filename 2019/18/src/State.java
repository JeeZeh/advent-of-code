import java.util.ArrayList;
import java.util.HashSet;
import java.util.Objects;

public class State {
    String c;
    HashSet<String> have;

    public State(String c, ArrayList<String> have) {
        this.c = c;
        this.have = new HashSet<>(have);
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) {
            return true;
        }
        if (o == null || getClass() != o.getClass()) {
            return false;
        }
        State state = (State) o;
        return c.equals(state.c) &&
                have.equals(state.have);
    }

    @Override
    public int hashCode() {
        return Objects.hash(c, have);
    }
}

