package exception;

import com.sun.source.tree.AssertTree;

/**
 * @author Zexho
 * @date 2021/5/14 3:57 下午
 */
public class ZexException extends RuntimeException{

    public ZexException(String m){super(m);}
//    public ZexException(String m, ASTree t){super(m + " " + t.localtion());}

}
