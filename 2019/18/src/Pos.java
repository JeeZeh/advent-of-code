import java.util.Objects;

public class Pos {
    int x;
    int y;

    public Pos(int x, int y) {
        this.x = x;
        this.y = y;
    }

    public Pos add(int a, int b) {
        this.x+=a;
        this.y+=b;
        return this;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) {
            return true;
        }
        if (o == null || getClass() != o.getClass()) {
            return false;
        }
        Pos pos = (Pos) o;
        return x == pos.x &&
                y == pos.y;
    }

    @Override
    public int hashCode() {
        return Objects.hash(x, y);
    }
}
