package aoc;

import java.util.List;
import java.util.Map;
import java.util.function.Consumer;
import java.util.function.Function;

import static java.util.Map.entry;
import lombok.Data;

@Data
class Operands {
    final int a;
    final int b;
    final int c;
}

interface Operation {
    void func(Operands ops);
}

public class CPU {
    int[] reg = { 0, 0, 0, 0 };

    final Map<String, Consumer<Operands>> operations = Map.ofEntries(
            entry("addr", this::addr),
            entry("addi", this::addi),
            entry("mulr", this::mulr),
            entry("muli", this::muli),
            entry("banr", this::banr),
            entry("bani", this::bani),
            entry("borr", this::borr),
            entry("bori", this::bori),
            entry("setr", this::setr),
            entry("seti", this::setr),
            entry("gtir", this::gtir),
            entry("gtri", this::gtri),
            entry("gtrr", this::gtrr),
            entry("eqir", this::eqir),
            entry("eqri", this::eqri),
            entry("eqrr", this::eqrr));

    void exampleCall() {
        Consumer<Operands> op = this.operations.get("addr");
        op.accept(new Operands(1, 2, 3));
    }

    void addr(Operands ops) {
        this.reg[ops.c] = this.reg[ops.a] + this.reg[ops.b];
    }

    void addi(Operands ops) {
        this.reg[ops.c] = this.reg[ops.a] + ops.b;
    }

    void mulr(Operands ops) {
        this.reg[ops.c] = this.reg[ops.a] * this.reg[ops.b];
    }

    void muli(Operands ops) {
        this.reg[ops.c] = this.reg[ops.a] * ops.b;
    }

    void banr(Operands ops) {
        this.reg[ops.c] = this.reg[ops.a] & this.reg[ops.b];
    }

    void bani(Operands ops) {
        this.reg[ops.c] = this.reg[ops.a] & ops.b;
    }

    void borr(Operands ops) {
        this.reg[ops.c] = this.reg[ops.a] | this.reg[ops.b];
    }

    void bori(Operands ops) {
        this.reg[ops.c] = this.reg[ops.a] | ops.b;
    }

    void setr(Operands ops) {
        this.reg[ops.c] = this.reg[ops.a];
    }

    void seti(Operands ops) {
        this.reg[ops.c] = ops.a;
    }

    void gtir(Operands ops) {
        this.reg[ops.c] = ops.a > this.reg[ops.b] ? 1 : 0;
    }

    void gtri(Operands ops) {
        this.reg[ops.c] = this.reg[ops.a] > ops.b ? 1 : 0;
    }

    void gtrr(Operands ops) {
        this.reg[ops.c] = this.reg[ops.a] > this.reg[ops.b] ? 1 : 0;
    }

    void eqir(Operands ops) {
        this.reg[ops.c] = ops.a == this.reg[ops.b] ? 1 : 0;
    }

    void eqri(Operands ops) {
        this.reg[ops.c] = this.reg[ops.a] == ops.b ? 1 : 0;
    }

    void eqrr(Operands ops) {
        this.reg[ops.c] = this.reg[ops.a] == this.reg[ops.b] ? 1 : 0;
    }

}
