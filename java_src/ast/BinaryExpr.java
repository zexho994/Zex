package ast;

import java.util.List;

/**
 * @author Zexho
 * @date 2021/5/17 12:36 下午
 */
public class BinaryExpr extends ASTList {

    public BinaryExpr(List<ASTree> list) {
        super(list);
    }

    public ASTree left() {
        return child(0);
    }

    public String operator() {
        return ((ASTLeaf) child(1)).token().getText();
    }

    public ASTree right() {
        return child(2);
    }
}
