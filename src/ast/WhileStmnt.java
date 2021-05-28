package ast;

import java.util.List;

/**
 * @author Zexho
 * @date 2021/5/28 8:35 上午
 */
public class WhileStmnt extends ASTList {
    public WhileStmnt(List<ASTree> c) {
        super(c);
    }

    public ASTree condition() {
        return child(0);
    }

    public ASTree body() {
        return child(1);
    }
}
