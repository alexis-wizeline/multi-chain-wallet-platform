import {
  ArgumentsHost,
  Catch,
  ExceptionFilter,
  HttpException,
  HttpStatus,
} from '@nestjs/common';

@Catch()
export class HttpExceptionFilert implements ExceptionFilter {
  catch(exception: any, host: ArgumentsHost) {
    const ctx = host.switchToHttp();
    const response = ctx.getResponse();
    const request = ctx.getRequest();

    if (exception instanceof HttpException) {
      const status = exception.getStatus();
      const res = exception.getResponse();

      if (typeof res === 'object' && res && 'error' in res) {
        return response.status(status).json(res);
      }

      return response.status(status).json({
        error: {
          code: 'HTTP_ERROR',
          message:
            typeof res === 'string'
              ? res
              : (res as any)?.message || 'Request Failed',
        },
      });
    }

    return response.status(HttpStatus.INTERNAL_SERVER_ERROR).json({
      error: {
        code: 'INTERNAL_ERROR',
        message: 'An unexpected error occurred',
        details: {
          path: request.url,
        },
      },
    });
  }
}
