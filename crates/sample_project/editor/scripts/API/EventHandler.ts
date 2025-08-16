/**
 * A specific event type that can be handled by the Events class.
 */
export type EventType = string;

/**
 * A specific event handler type that can be used to handle events.
 */
export type EventHandler = (...args: any[]) => Promise<void>;

/**
 * This class can be inherited and used to handle event callbacks.
 */
export class Events {
  private onHandlers: { [id: EventType]: EventHandler[] } = {};
  private onceHandlers: { [id: EventType]: EventHandler[] } = {};
  private toRegisterOn: [EventType, EventHandler][] = [];
  private toRegisterOnce: [EventType, EventHandler][] = [];
  private toRemove: EventHandler[] = [];
  private isEmitting = false;

  /**
   * Call an event handler, if it exists. Events are called in the order they
   * were registered, with temporary handlers always being called last.
   * @param event The event to call.
   * @param  args The arguments to pass to the event handler.
   */
  public async emit(event: EventType, ...args: any[]): Promise<void> {
    let wasEmitting = this.isEmitting;
    this.isEmitting = true;

    if (this.onHandlers[event]) {
      for (const handler of this.onHandlers[event]) {
        await handler(...args);
      }
    }

    if (this.onceHandlers[event]) {
      for (const handler of this.onceHandlers[event]) {
        await handler(...args);
      }
      delete this.onceHandlers[event];
    }

    this.isEmitting = wasEmitting;
    if (!this.isEmitting) {
      for (const [event, handler] of this.toRegisterOn) {
        this.on(event, handler);
      }
      this.toRegisterOn = [];

      for (const [event, handler] of this.toRegisterOnce) {
        this.once(event, handler);
      }
      this.toRegisterOnce = [];

      for (const handler of this.toRemove) {
        this.removeListener(handler);
      }
      this.toRemove = [];
    }
  }

  /**
   * Registers a new event handler for the given event. Multiple handlers can
   * be registered for the same event.
   *
   * Any event handlers registered while an event is being emitted will not be
   * called during the current event emission, and only be called during the
   * next event emission.
   * @param event The event to set the handler for.
   * @param handler The handler to set. May be async.
   */
  public on(event: EventType, handler: EventHandler): void {
    if (this.isEmitting) {
      this.toRegisterOn.push([event, handler]);
      return;
    }

    if (!this.onHandlers[event]) {
      this.onHandlers[event] = [];
    }

    this.onHandlers[event].push(handler);
  }

  /**
   * Registers a new event handler for the given event that will only be called
   * once. The handler will be removed after it is called. Multiple handlers can
   * be registered for the same event.
   *
   * Any event handlers registered while an event is being emitted will not be
   * called during the current event emission, and only be called during the
   * next event emission.
   * @param event The event to set the handler for.
   * @param handler The handler to set. May be async.
   */
  public once(event: EventType, handler: EventHandler): void {
    if (this.isEmitting) {
      this.toRegisterOn.push([event, handler]);
      return;
    }

    if (!this.onceHandlers[event]) {
      this.onceHandlers[event] = [];
    }

    this.onceHandlers[event].push(handler);
  }

  /**
   * Waits for the given event to be called. This is a convenience function that
   * creates a promise that resolves when the event is called, returning the
   * arguments passed to the event handler.
   * @param event The event to wait for.
   * @returns A promise that resolves when the event is called. Returns the
   * arguments passed to the event handler.
   */
  public async waitFor(event: EventType): Promise<any[]> {
    return await new Promise((resolve) => {
      this.once(event, async (...args) => resolve(args));
    });
  }

  /**
   * Removes the given handler from the event emitter. If the handler does not
   * exist, this function does nothing. If this function is called while an
   * event is being emitted, the handler will be removed after the current event
   * emission completes.
   * @param handler The handler to remove.
   */
  public removeListener(handler: EventHandler) {
    if (this.isEmitting) {
      this.toRemove.push(handler);
      return;
    }

    for (const event in this.onHandlers) {
      this.onHandlers[event] = this.onHandlers[event].filter(
        (h) => h !== handler
      );
    }

    for (const event in this.onceHandlers) {
      this.onceHandlers[event] = this.onceHandlers[event].filter(
        (h) => h !== handler
      );
    }
  }
}
