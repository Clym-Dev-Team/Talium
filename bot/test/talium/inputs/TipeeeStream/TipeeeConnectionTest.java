package talium.inputs.TipeeeStream;

import org.junit.jupiter.api.Test;
import org.mockito.Mockito;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.test.context.ActiveProfiles;
import talium.system.inputSystem.HealthManager;
import talium.system.inputSystem.InputStatus;

import java.time.Instant;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.fail;

@SpringBootTest
@ActiveProfiles("test")
public class TipeeeConnectionTest {
    @Autowired
    TipeeeConfig config;

    @Test
    void connect() {
        var donationRepo = Mockito.mock(DonationRepo.class);
        try {
            var tipeee = new TipeeeInput(config, donationRepo);
            var shouldHaveFinishedBy = Instant.now().plusSeconds(10);
            tipeee.startup();
            while (HealthManager.get(TipeeeInput.class) == InputStatus.STARTING && shouldHaveFinishedBy.isAfter(Instant.now())) {
                Thread.onSpinWait();
            }
            assertEquals(InputStatus.HEALTHY, HealthManager.get(TipeeeInput.class));
            tipeee.shutdown();
        } catch (TipeeeInput.TipeeeStreamException e) {
            fail("Connection failed with: ", e);
        }
    }
}
