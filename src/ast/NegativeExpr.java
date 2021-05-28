package ast;

import java.util.List;

/**
 * @author Zexho
 * @date 2021/5/28 8:34 上午
 */
public class NegativeExpr extends ASTList {
    public NegativeExpr(List<ASTree> c) {
        super(c);
    }

    public ASTree operand() {
        return child(0);
    }
}