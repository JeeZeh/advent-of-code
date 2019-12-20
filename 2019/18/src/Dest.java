import java.util.ArrayList;
import java.util.List;
import java.util.Objects;

public class Dest {
    String key;
    int dist;
    List<String> req;

    public Dest(String key, int dist, List<String> req) {
        this.key = key;
        this.dist = dist;
        this.req = req;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        Dest dest = (Dest) o;
        return dist == dest.dist &&
                key.equals(dest.key) &&
                req.equals(dest.req);
    }

    @Override
    public int hashCode() {
        return Objects.hash(key, dist, req);
    }
}
