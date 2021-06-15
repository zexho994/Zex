package ast;

import java.util.List;

/**
 * @author Zexho
 * @date 2021/5/28 8:33 上午
 */
public class IfStmnt extends ASTList {
    public IfStmnt(List<ASTree> c) {
        super(c);
    }

    public ASTree condition() {
        return child(0);
    }

    public ASTree thenBlock() {
        return child(1);
    }

    public ASTree elseBlock() {
        return numChildren() > 2 ? child(2) : null;
    }
}
