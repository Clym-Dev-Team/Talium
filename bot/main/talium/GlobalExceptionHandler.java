package talium;

import com.google.gson.JsonSyntaxException;
import org.springframework.http.HttpStatus;
import org.springframework.web.bind.annotation.ExceptionHandler;
import org.springframework.web.bind.annotation.ResponseStatus;
import org.springframework.web.bind.annotation.RestControllerAdvice;
import org.springframework.web.servlet.mvc.method.annotation.ResponseEntityExceptionHandler;

@RestControllerAdvice
public class GlobalExceptionHandler extends ResponseEntityExceptionHandler {

    @ResponseStatus(HttpStatus.INTERNAL_SERVER_ERROR)
    @ExceptionHandler(value = {Exception.class})
    public String generalException(Exception e) {
        logger.error(e.getMessage(), e);
        return "An Exception occurred!";
    }

    @ResponseStatus(HttpStatus.BAD_REQUEST)
    @ExceptionHandler(value = {JsonSyntaxException.class})
    public String deserializationException(Exception e) {
        logger.error(e.getMessage(), e);
        return "{ \"error\": \"Failed to deserialize json\", \"message\": \"" + e.getMessage() + "\" }";
    }
}
